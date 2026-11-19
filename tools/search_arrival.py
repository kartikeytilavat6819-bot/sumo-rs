import os
base=r'C:\Users\karti\sumo-src\src'
for root,_,files in os.walk(base):
    for f in files:
        if not f.endswith(('.cpp','.h')):
            continue
        p=os.path.join(root,f)
        try:
            s=open(p,errors='ignore').read()
        except Exception:
            continue
        if 'arrivalPos' in s and 'random' in s:
            print(p)
print('done')
