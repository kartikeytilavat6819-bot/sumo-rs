import os
base=r'C:\Users\karti\sumo-rs\.claude\worktrees\agent-oc87\tools\oracle\cases'
count=0
for name in os.listdir(base):
    if not name.startswith('duarouter_'):
        continue
    d=os.path.join(base,name)
    if not os.path.isdir(d):
        continue
    for f in os.listdir(d):
        p=os.path.join(d,f)
        if not os.path.isfile(p):
            continue
        try:
            s=open(p,errors='ignore').read()
        except Exception:
            continue
        if 'No output specified' in s:
            print(p)
            count+=1
print("done", count)
