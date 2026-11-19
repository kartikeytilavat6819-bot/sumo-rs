p='crates/sumo_core/src/router/ro_route_handler.rs'
s=open(p,encoding='utf-8').read()
old='''        let offsetToBegin = self.myBegin - self.myVehicleParameter.depart;
        while self.myVehicleParameter.repetitionTotalOffset < offsetToBegin {'''
new='''        let offsetToBegin = self.myBegin - self.myVehicleParameter.depart;
        eprintln!("DEBUG close_flow myBegin={} depart={} offsetToBegin={} repNum={} repOffset={} repTotalOffset={}", self.myBegin, self.myVehicleParameter.depart, offsetToBegin, self.myVehicleParameter.repetitionNumber, self.myVehicleParameter.repetitionOffset, self.myVehicleParameter.repetitionTotalOffset);
        while self.myVehicleParameter.repetitionTotalOffset < offsetToBegin {'''
assert old in s, "old not found"
s=s.replace(old,new)
open(p,'w',encoding='utf-8').write(s)
print("done")
