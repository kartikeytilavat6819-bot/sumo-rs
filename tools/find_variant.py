import os
base=r'C:\Users\karti\sumo-src\tests\duarouter'
print(os.path.exists(base))
for root,dirs,files in os.walk(base):
    ndots=[f for f in files if '.' not in f]
    if ndots:
        print(repr(root), ndots)
print("done")
