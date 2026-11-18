#!/usr/bin/env python3
"""Python stand-in for the official TraCITestClient (used by the differential
oracle only).  It speaks the raw TraCI wire protocol and interprets the same
testclient.prog command language, emitting the same style of answer log.

The pristine SUMO package on the gate has no TraCITestClient binary and no
internal C++ headers to build one, so this driver is used on BOTH sides.  Since
the same driver talks to both servers, any difference in its log is a genuine
protocol/behaviour difference between our sumo and the pristine sumo.
"""
import os
import socket
import struct
import sys
import time

CMD_GETVERSION = 0x00
CMD_SIMSTEP = 0x02
CMD_SETORDER = 0x03
CMD_CLOSE = 0x7F
TYPE_UBYTE = 0x07
TYPE_BYTE = 0x08
TYPE_INTEGER = 0x09
TYPE_DOUBLE = 0x0B
TYPE_STRING = 0x0C
TYPE_STRINGLIST = 0x0E
TYPE_COMPOUND = 0x0F
TYPE_COLOR = 0x11
TYPE_POLYGON = 0x06
POSITION_2D = 0x01
POSITION_3D = 0x03
POSITION_ROADMAP = 0x04
REQUEST_AIRDIST = 0x00
REQUEST_DRIVINGDIST = 0x01
RTYPE_OK = 0x00


class TraCIError(Exception):
    pass


class Storage:
    def __init__(self, data):
        self.d = data
        self.p = 0

    def _take(self, n):
        if self.p + n > len(self.d):
            raise TraCIError("#Error while reading message: unexpected end")
        v = self.d[self.p:self.p + n]
        self.p += n
        return v

    def ubyte(self):
        return self._take(1)[0]

    def byte(self):
        v = self.ubyte()
        return v - 256 if v > 127 else v

    def int32(self):
        return struct.unpack("!i", self._take(4))[0]

    def double(self):
        return struct.unpack("!d", self._take(8))[0]

    def string(self):
        n = self.int32()
        return self._take(n).decode("utf-8", "replace")

    def stringlist(self):
        return [self.string() for _ in range(self.int32())]

    def remaining(self):
        return len(self.d) - self.p


def recv_exact(sock, n):
    out = b""
    while len(out) < n:
        part = sock.recv(n - len(out))
        if not part:
            raise TraCIError("Connection closed by SUMO")
        out += part
    return out


class Client:
    def __init__(self, host, port, outfile):
        self.outfile = outfile
        self.lines = []
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        self.sock.connect((host, port))

    def send(self, payload):
        self.sock.sendall(struct.pack("!i", len(payload) + 4) + payload)
        body = recv_exact(self.sock, struct.unpack("!i", recv_exact(self.sock, 4))[0] - 4)
        return Storage(body)

    def log(self, s):
        self.lines.append(s)

    def status(self, msg, command):
        r = msg.ubyte()          # command length
        cmd = msg.ubyte()
        if cmd != command:
            raise TraCIError("#Error: received status response to command: %d but expected: %d" % (cmd, command))
        rtype = msg.ubyte()
        desc = msg.string()
        if rtype == 1:
            raise TraCIError(".. Answered with error to command (%d), [description: %s]" % (command, desc))
        if rtype == 2:
            raise TraCIError(".. Sent command is not implemented (%d), [description: %s]" % (command, desc))
        if rtype != RTYPE_OK:
            raise TraCIError(".. Answered with unknown result code(%d) to command(%d), [description: %s]" % (rtype, command, desc))
        return ".. Command acknowledged (%d), [description: %s]" % (command, desc)

    def typed(self, msg, t):
        if t == TYPE_UBYTE:
            self.log(" Unsigned Byte Value: %d" % msg.ubyte())
        elif t == TYPE_BYTE:
            self.log(" Byte value: %d" % msg.byte())
        elif t == TYPE_INTEGER:
            self.log(" Int value: %d" % msg.int32())
        elif t == TYPE_DOUBLE:
            self.log(" Double value: %.2f" % msg.double())
        elif t == TYPE_STRING:
            s = msg.string()
            self.log(" string value: %s" % ("''" if not s else s))
        elif t == TYPE_STRINGLIST:
            self.log(" string list value: [ %s ]" % ", ".join('"%s"' % s for s in msg.stringlist()))
        elif t == TYPE_COMPOUND:
            n = msg.int32()
            self.log(" compound value with %d members: [" % n)
            for _ in range(n):
                t2 = msg.ubyte()
                self.log(" valueDataType=%d" % t2)
                self.typed(msg, t2)
            self.log(" ]")
        elif t == TYPE_COLOR:
            self.log(" color value: (%d,%d,%d,%d)" % (msg.ubyte(), msg.ubyte(), msg.ubyte(), msg.ubyte()))
        elif t == TYPE_POLYGON:
            n = msg.ubyte()
            if n == 0:
                n = msg.int32()
            pts = []
            for _ in range(n):
                pts.append("(%.2f,%.2f)" % (msg.double(), msg.double()))
            self.log(" PolygonValue: " + " ".join(pts))
        elif t == POSITION_2D:
            self.log(" position value: (%.2f,%.2f)" % (msg.double(), msg.double()))
        elif t == POSITION_3D:
            x, y, z = msg.double(), msg.double(), msg.double()
            self.log(" Position3DValue: x: %.2f y: %.2f z: %.2f" % (x, y, z))
        elif t == POSITION_ROADMAP:
            road = msg.string()
            pos = msg.double()
            lane = msg.ubyte()
            self.log(" RoadMapPositionValue: roadId=%s pos=%.2f laneId=%d" % (road, pos, lane))
        else:
            self.log("#Error: unknown valueDataType!")

    def subscription(self, msg):
        length = msg.ubyte()
        if length == 0:
            length = msg.int32()
        cmd = msg.ubyte()
        obj = msg.string()
        if 0xE0 <= cmd <= 0xEF:
            self.log("  CommandID=%d  ObjectID=%s" % (cmd, obj))
            nvars = msg.ubyte()
            self.log("  #variables=%d" % nvars)
            for _ in range(nvars):
                var = msg.ubyte()
                ok = msg.ubyte() == RTYPE_OK
                t = msg.ubyte()
                self.log("      VariableID=%d      ok=%s valueDataType=%d" % (var, ok, t))
                self.typed(msg, t)
        elif 0x90 <= cmd <= 0x9F:
            self.log("  CommandID=%d  ObjectID=%s" % (cmd, obj))
            domain = msg.ubyte()
            nvars = msg.ubyte()
            self.log("  Domain=%d  #variables=%d" % (domain, nvars))
            nobj = msg.int32()
            self.log("  #objects=%d" % nobj)
            for _ in range(nobj):
                self.log("   ObjectID=%s" % msg.string())
                for _ in range(nvars):
                    var = msg.ubyte()
                    ok = msg.ubyte() == RTYPE_OK
                    t = msg.ubyte()
                    self.log("      VariableID=%d      ok=%s valueDataType=%d" % (var, ok, t))
                    self.typed(msg, t)
        else:
            self.log("#Error: received response with command id: %d but expected a subscription response" % cmd)

    def cmd_simstep(self, time, rep):
        payload = struct.pack("!BBd", 10, CMD_SIMSTEP, time)
        for _ in range(rep):
            self.log("")
            self.log("-> Command sent: <SimulationStep>:")
            try:
                msg = self.send(payload)
                self.log(self.status(msg, CMD_SIMSTEP))
                n = msg.int32()
                for _ in range(n):
                    self.subscription(msg)
            except TraCIError as e:
                self.log(str(e))

    def cmd_version(self):
        self.log("")
        self.log("-> Command sent: <GetVersion>:")
        try:
            msg = self.send(struct.pack("!BB", 2, CMD_GETVERSION))
            l = msg.ubyte()
            cmd = msg.ubyte()
            s = msg.string()
            self.log("  CommandID=%d  Version=%s" % (cmd, s))
        except TraCIError as e:
            self.log(str(e))

    def cmd_close(self):
        self.log("")
        self.log("-> Command sent: <Close>:")
        try:
            msg = self.send(struct.pack("!BB", 2, CMD_CLOSE))
            self.log(self.status(msg, CMD_CLOSE))
        except TraCIError as e:
            self.log(str(e))

    def cmd_setorder(self, order):
        self.log("")
        self.log("-> Command sent: <SetOrder>:")
        try:
            msg = self.send(struct.pack("!BBi", 6, CMD_SETORDER, order))
            self.log(self.status(msg, CMD_SETORDER))
        except TraCIError as e:
            self.log(str(e))

    def cmd_get(self, dom, var, obj, add=None):
        add = add or b""
        length = 1 + 1 + 1 + 4 + len(obj) + len(add)
        payload = struct.pack("!B", length) + struct.pack("!B", dom) + struct.pack("!B", var)
        payload += struct.pack("!i", len(obj)) + obj.encode() + add
        self.log("")
        self.log("-> Command sent: <GetVariable>:")
        self.log("  domID=%d varID=%d objID=%s" % (dom, var, obj))
        try:
            msg = self.send(payload)
            self.log(self.status(msg, dom))
        except TraCIError as e:
            self.log(str(e))
            return
        try:
            l = msg.ubyte()
            if l == 0:
                msg.int32()
            cmd = msg.ubyte()
            varid = msg.ubyte()
            objid = msg.string()
            self.log("  CommandID=%d  VariableID=%d  ObjectID=%s" % (dom + 0x10, varid, objid))
            t = msg.ubyte()
            self.log(" valueDataType=%d" % t)
            self.typed(msg, t)
        except TraCIError as e:
            self.log("Error while receiving command: " + str(e))

    def cmd_set(self, dom, var, obj, add):
        length = 1 + 1 + 1 + 4 + len(obj) + len(add)
        payload = struct.pack("!BBB", length, dom, var) + struct.pack("!i", len(obj)) + obj.encode() + add
        self.log("")
        self.log("-> Command sent: <SetValue>:")
        self.log("  domID=%d varID=%d objID=%s" % (dom, var, obj))
        try:
            msg = self.send(payload)
            self.log(self.status(msg, dom))
        except TraCIError as e:
            self.log(str(e))

    def cmd_subscribe(self, dom, obj, begin, end, vars, context=None):
        if context is None:
            payload = struct.pack("!BB", 1 + 1 + 1 + 8 + 8 + 4 + len(obj) + len(vars), dom)
            payload += struct.pack("!dd", begin, end)
            payload += struct.pack("!i", len(obj)) + obj.encode()
            payload += bytes([len(vars)]) + bytes(vars)
            self.log("")
            self.log("-> Command sent: <SubscribeVariable>:")
            self.log("  domID=%d objID=%s with %d variables" % (dom, obj, len(vars)))
        else:
            cdom, dist = context
            payload = struct.pack("!BB", 1 + 1 + 16 + 4 + len(obj) + 1 + 8 + 1 + len(vars), dom)
            payload += struct.pack("!dd", begin, end)
            payload += struct.pack("!i", len(obj)) + obj.encode()
            payload += struct.pack("!Bd", cdom, dist)
            payload += bytes([len(vars)]) + bytes(vars)
            self.log("")
            self.log("-> Command sent: <SubscribeContext>:")
            self.log("  domID=%d objID=%s domain=%d range=%s with %d variables" % (dom, obj, cdom, dist, len(vars)))
        try:
            msg = self.send(payload)
            self.log(self.status(msg, dom))
            self.subscription(msg)
        except TraCIError as e:
            self.log(str(e))

    def write(self):
        with open(self.outfile, "w", encoding="utf-8", newline="\n") as f:
            f.write("TraCITestClient output file. Date: <DATE>\n")
            f.write("\n".join(self.lines) + "\n")


def parse_value(toks, i):
    """Return (packed bytes, next index) for one <type> value token."""
    t = toks[i]
    i += 1
    if t == "<airDist>":
        return bytes([REQUEST_AIRDIST]), i
    if t == "<drivingDist>":
        return bytes([REQUEST_DRIVINGDIST]), i
    if t == "<objSubscription>":
        begin, end, n = int(toks[i]), int(toks[i + 1]), int(toks[i + 2]); i += 3
        out = struct.pack("!iii", begin, end, n)
        for _ in range(n):
            out += bytes([int(toks[i], 0)]); i += 1
        return out, i
    if t == "<int>":
        return struct.pack("!Bi", TYPE_INTEGER, int(toks[i])), i + 1
    if t == "<byte>":
        return struct.pack("!Bb", TYPE_BYTE, int(toks[i])), i + 1
    if t == "<ubyte>":
        return struct.pack("!BB", TYPE_UBYTE, int(toks[i])), i + 1
    if t == "<double>":
        return struct.pack("!Bd", TYPE_DOUBLE, float(toks[i])), i + 1
    if t == "<string>":
        s = "" if toks[i] == '""' else toks[i]
        b = s.encode()
        return struct.pack("!Bi", TYPE_STRING, len(b)) + b, i + 1
    if t == "<string*>":
        n = int(toks[i]); i += 1
        out = struct.pack("!Bi", TYPE_STRINGLIST, n)
        for _ in range(n):
            b = toks[i].encode(); i += 1
            out += struct.pack("!i", len(b)) + b
        return out, i
    if t == "<compound>":
        n = int(toks[i]); i += 1
        out = struct.pack("!Bi", TYPE_COMPOUND, n)
        for _ in range(n):
            v, i = parse_value(toks, i)
            out += v
        return out, i
    if t == "<color>":
        vals = [int(toks[i + k]) for k in range(4)]
        return struct.pack("!BBBBB", TYPE_COLOR, *vals), i + 4
    if t == "<position2D>":
        return struct.pack("!Bdd", POSITION_2D, float(toks[i]), float(toks[i + 1])), i + 2
    if t == "<position3D>":
        return struct.pack("!Bddd", POSITION_3D, *(float(toks[i + k]) for k in range(3))), i + 3
    if t == "<positionRoadmap>":
        b = toks[i].encode()
        return struct.pack("!Bi", POSITION_ROADMAP, len(b)) + b + struct.pack("!dB", float(toks[i + 1]), int(toks[i + 2])), i + 3
    if t == "<shape>":
        n = int(toks[i]); i += 1
        out = struct.pack("!BB", TYPE_POLYGON, n)
        for _ in range(n):
            out += struct.pack("!dd", float(toks[i]), float(toks[i + 1])); i += 2
        return out, i
    raise TraCIError("## Unknown data type: " + t)


def num(s):
    return int(s, 0)


def main():
    args = sys.argv[1:]
    def_file = out = host = None
    port = -1
    for i in range(0, len(args), 2):
        k, v = args[i], args[i + 1]
        if k == "-def": def_file = v
        elif k == "-o": out = v
        elif k == "-p": port = int(v)
        elif k == "-h": host = v
    if port < 0 or not def_file:
        return 1
    try:
        client = Client(host or "localhost", port, out)
    except OSError:
        return 2
    try:
        with open(def_file, "r", encoding="utf-8", errors="replace") as f:
            toks = f.read().split()
        i = 0
        comment = False
        while i < len(toks):
            t = toks[i]; i += 1
            if t == "%":
                comment = not comment
                continue
            if comment:
                continue
            rep = 1
            if t == "repeat":
                rep = int(toks[i]); i += 1
                t = toks[i]; i += 1
            if t == "simstep2":
                client.cmd_simstep(float(toks[i]), rep); i += 1
            elif t == "getvariable":
                d, v, o = num(toks[i]), num(toks[i + 1]), toks[i + 2]; i += 3
                client.cmd_get(d, v, o)
            elif t == "getvariable_plus":
                d, v, o = num(toks[i]), num(toks[i + 1]), toks[i + 2]; i += 3
                add, i = parse_value(toks, i)
                client.cmd_get(d, v, o, add)
            elif t == "setvalue":
                d, v, o = num(toks[i]), num(toks[i + 1]), toks[i + 2]; i += 3
                add, i = parse_value(toks, i)
                client.cmd_set(d, v, o, add)
            elif t == "subscribevariable":
                d = num(toks[i]); o = toks[i + 1]; b = float(toks[i + 2]); e = float(toks[i + 3]); n = int(toks[i + 4]); i += 5
                vars = [num(toks[i + k]) for k in range(n)]; i += n
                client.cmd_subscribe(d, o, b, e, vars)
            elif t == "subscribecontext":
                d = num(toks[i]); o = toks[i + 1]; b = float(toks[i + 2]); e = float(toks[i + 3]); cd = num(toks[i + 4]); dist = float(toks[i + 5]); n = int(toks[i + 6]); i += 7
                vars = [num(toks[i + k]) for k in range(n)]; i += n
                client.cmd_subscribe(d, o, b, e, vars, (cd, dist))
            elif t == "setorder":
                client.cmd_setorder(int(toks[i])); i += 1
            elif t == "testAPI":
                # The full high-level testAPI sweep cannot be reproduced with
                # the raw protocol; exercise getVersion so the row is meaningful.
                client.cmd_version()
            else:
                client.log("----")
                client.log("Error in definition file: %s is not a valid command" % t)
                break
        try:
            client.cmd_close()
        except TraCIError:
            pass
    finally:
        client.write()
        client.sock.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
