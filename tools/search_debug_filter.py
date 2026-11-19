import os
base=r'C:\Users\karti\sumo-src\tests\duarouter'
for root,_,files in os.walk(base):
    for f in files:
        p=os.path.join(root,f)
        try:
            s=open(p,errors='ignore').read()
        except Exception:
            continue
        if 'debug_filter' in s:
            print(p)
print('done')
