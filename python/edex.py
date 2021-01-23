import json
import sys

query="data"

json_file = open('data/hist.json')

print(sys.argv)
data = json.load(json_file)
for i in range(1, len(sys.argv)):
    for j in range(eval("len("+query+")")):
        if eval(query+"[j]['command'] == sys.argv[i]"):
            query+="["+str(j)+"]['sub']"
            print(j)
            break
if 
exec(query+".append({\'command\':\'"+sys.argv[-1]+"\', \'sub\':[]})")
json_file.close()
with open('data/hist.json', 'w') as outfile:
    json.dump(data, outfile)