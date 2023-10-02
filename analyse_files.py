from os import system,listdir,path as chr

def search(path:str = "", exclude:list[str] = [], debug:bool = False) -> list[str]:
	output = []
	current = ""
	enter_folder = True
	while 1:
		dossier = listdir(path if path not in ["/",""] else None)
		try:
			if enter_folder:
				enter_folder = False
				current = dossier[0]
			else: current = dossier[dossier.index(current) + 1]
		except IndexError:
			temp = path.split("/")
			path = path[:-len(temp[-2])-1]
			current = temp[-2]
			enter_folder = False
			if debug: print(path, current)
			continue
		if path + current not in exclude:
			if chr.isdir(path + current):
				path += current + "/"
				enter_folder = True
			else:
				output.append(path + current)
				if debug: print(path + current)
		if current == dossier[-1] and path == "": return output
def path_list_to_dir_dict(paths:list[str],debug:bool = False):
	out = {}
	for path in paths:
		temp = [out]
		files = path.split("/")
		for file in range(len(files)):
			if debug:
				system("clear")
				print(out)
			if file == len(files)-1:
				temp[-1][files[file]] = files[file].split(".")[-1]
			else:
				try:temp[-1][files[file]]
				except KeyError:
					temp[-1][files[file]] = {}
				temp.append(temp[-1][files[file]])
	return out

