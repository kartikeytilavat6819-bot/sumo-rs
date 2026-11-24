#!/usr/bin/env bash
# oc122 probe (temporary): run the official wheel's libproj on the polyconvert tie coordinates
LIB=$(ls ${SUMO_PRISTINE_BIN}/../../eclipse_sumo.libs/libproj-*.so* 2>/dev/null | head -1)
echo "lib: $LIB"
strings -a "$LIB" | grep -m3 -E "^[0-9]+\.[0-9]+\.[0-9]+$|Rel\. [0-9]"
python3 - "$LIB" <<'PY'
import ctypes, math, sys
L=ctypes.CDLL(sys.argv[1])
class V(ctypes.Structure): _fields_=[("major",ctypes.c_int),("minor",ctypes.c_int),("patch",ctypes.c_int),("release",ctypes.c_char_p),("version",ctypes.c_char_p),("searchpath",ctypes.c_char_p),("paths",ctypes.c_void_p),("path_count",ctypes.c_size_t)]
L.proj_info.restype=V
i=L.proj_info(); print("proj", i.major,i.minor,i.patch, i.version)
class C(ctypes.Structure): _fields_=[("v",ctypes.c_double*4)]
L.proj_create.restype=ctypes.c_void_p; L.proj_create.argtypes=[ctypes.c_void_p,ctypes.c_char_p]
L.proj_trans.restype=C; L.proj_trans.argtypes=[ctypes.c_void_p,ctypes.c_int,C]
L.proj_torad.restype=ctypes.c_double; L.proj_torad.argtypes=[ctypes.c_double]
L.proj_todeg.restype=ctypes.c_double; L.proj_todeg.argtypes=[ctypes.c_double]
P=L.proj_create(None,b"+proj=utm +zone=33 +ellps=WGS84 +datum=WGS84 +units=m +no_defs")
f=lambda a,b: L.proj_trans(P,1,C((ctypes.c_double*4)(a,b,0,0))).v[:2]
g=lambda a,b: L.proj_trans(P,-1,C((ctypes.c_double*4)(a,b,0,0))).v[:2]
tests=[("52.4284985","13.5275234","52.428498"),("52.4282635","13.5299562","52.428264"),("52.4262055","13.5200856","52.426206"),("52.4273049","13.5307795","13.530779")]
for mode in ("pi","proj"):
    out=[]
    for la,lo,_ in tests:
        lo=float(lo);la=float(la)
        r=(lo*math.pi/180.,la*math.pi/180.) if mode=="pi" else (L.proj_torad(lo),L.proj_torad(la))
        x,y=f(*r); l,p=g(x,y)
        l,p=(l*180./math.pi,p*180./math.pi) if mode=="pi" else (L.proj_todeg(l),L.proj_todeg(p))
        out.append("%.6f,%.6f"%(l,p)); 
    print(mode,out,[t[2] for t in tests])
print("fwd xy of tie1: %r"%(f(13.5275234*math.pi/180.,52.4284985*math.pi/180.),))
PY
echo "AGENT RUN DONE"
