csvfile = open("yield_df.csv", "r")
data = []
for row in csvfile.readlines():
    if "India" in row and "\"" not in row:
        i = 0
        temp = []
        for fields in row.split(","):
            i += 1
            if i in [1, 2, 4]:
                continue
            if i == 8:
                fields = fields[:-1]
            temp.append(fields)
        data.append(temp)
crop_encoding = {}
i = 0
for row in data:
    crop = row[0]
    if crop not in crop_encoding:
        crop_encoding [crop] = i
        i += 1
for i in range(len(data)):
    for j in range(len(data[0])):
        if j == 0:
            data[i][j] = crop_encoding[data[i][j]]
            continue
        data[i][j] = int(float(data[i][j]))

out_file = open("preprocessed.dat", "w")
out_file.flush()
for row in data:
    out_str = ""
    for field in row:
        out_str += f"{field} "
    out_file.write(f"{out_str[:-1]}\n")

mapping_file = open("mappings.dat", "w")
mapping_file.flush()
mapping_file.write(f"{crop_encoding}\n")
mapping_file.write("Crop Type\n")
mapping_file.write("Yield\n")
mapping_file.write("Average Rainfall\n")
mapping_file.write("Pesticide Usage\n")
mapping_file.write("Average Temperature\n")
