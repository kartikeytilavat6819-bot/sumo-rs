import glob, os
count=0
for p in glob.glob('/c/Users/karti/sumo-src/tests/**/testsuite*', recursive=True):
    try:
        s=open(p,errors='ignore').read()
    except Exception:
        continue
    if 'filter' in s.lower():
        print(p)
        count+=1
print("filter testsuites:", count)
# dirs named filter*
for root,dirs,files in os.walk('/c/Users/karti/sumo-src/tests'):
    for d in dirs:
        if 'filter' in d.lower():
            print("DIR:", os.path.join(root,d))
print("done")
