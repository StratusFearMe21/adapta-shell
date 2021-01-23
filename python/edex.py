import json
import sys

query="data"

json_file = open('data/hist.json')

print(sys.argv)

data = json.load(json_file)
if sys.argv[-1].startswith("-"):
    for j in range(eval("len("+query+")")):
        if eval(query+"[j]['command'] == sys.argv[1]"):
            query+="["+str(j)+"]['opts']"
            print(j)
            break
    exec(query+".append(\'"+sys.argv[-1]+"\')")
else:
    try:
        for i in range(1, len(sys.argv)):
            for j in range(eval("len("+query+")")):
                if eval(query+"[j]['command'] == sys.argv[i]"):
                    query+="["+str(j)+"]['sub']"
                    print(j)
                    break
        if len(sys.argv) <= 2:
            exec(query+".append({\'command\':\'"+sys.argv[-1]+"\', \'sub\':[], \'opts\':[]})")
        else:
            exec(query+".append({\'command\':\'"+sys.argv[-1]+"\', \'sub\':[]})")
    except KeyError:
        query="data"
        exec(query+".append({\'command\':\'"+sys.argv[-1]+"\', \'sub\':[], \'opts\':[]})")
        del data[0]

json_file.close()
with open('data/hist.json', 'w') as outfile:
    json.dump(data, outfile)