import os
import sys
import matplotlib
import matplotlib.pyplot as plt

matplotlib.use('Agg')

def parse_line(line:str) -> tuple[int,float,int,str]:
	data = line.split(":")


	id = int(data[0])
	timestamp = float(data[1])
	tries = int(data[2])
	tp = data[3]

	return (id, timestamp, tries, tp.strip())



def parse_case_terms_and_name():
	terms_map = {}
	i = 0
	name = ""

	for line in sys.stdin:
		if i == 0:
			name = line.strip()
			i += 1
			continue

		id, timestamp, tries, tp = parse_line(line)

		if id not in terms_map:
			terms_map[id] = {}

		if tp not in terms_map[id]:
			terms_map[id][tp] = {'id':id}
		
		if tries != -1:
			terms_map[id][tp]['end'] = timestamp
		else:
			terms_map[id][tp]['start'] = timestamp

	sys.stdout.flush()

	return terms_map, name



def analyze_gannt(terms_map, name, path):
	plt.figure()
	values = terms_map.values()
	sorted_values = sorted(values, key=lambda term: term['client']['start'])

	min_start = sorted_values[0]['client']['start']
	max_end = max([sorted_values[i]['client']['end'] for i in range(len(sorted_values))])

	client_color = 'red'
	server_color = 'blue'

	for task in sorted_values:
		id = task['client']['id']
		client_start = task['client']['start'] - min_start
		client_end = task['client']['end'] - min_start

		server_start = task['server']['start'] - min_start
		server_end = task['server']['end'] - min_start

		# print("")
		# print(f"client_start:{client_start}")
		# print(f"server_start:{server_start}")
		# print(f"server_end:{server_end}")
		# print(f"client_end:{client_end}")


		plt.barh(y=id, width=server_start - client_start, left=client_start, color=client_color)
		plt.barh(y=id, width=server_end - server_start, left=server_start, color=server_color)
		plt.barh(y=id, width=client_end - server_end, left=server_end, color=client_color)


	plt.legend([plt.Line2D([0], [0], color=client_color, lw=4), 
                plt.Line2D([0], [0], color=server_color, lw=4)], 
               ['Client', 'Server'], loc='upper right')

	plt.xlim(0, max_end - min_start)
	plt.yticks(list(terms_map.keys()))
	plt.xlabel("Time (ms since first task start)")
	plt.ylabel("Tasks")
	plt.grid(axis="x", linestyle="--", alpha=0.6)
	plt.tight_layout()
	plt.savefig(path)


def analyze_time_plot(terms_map, name, path, tp):
	plt.figure()
	values = [v[tp] for v in terms_map.values()]
	sorted_values = sorted(values, key=lambda term: term['end'])

	lasted_for = []

	for term in sorted_values:
		lasted_for.append(term['end'] - term['start'])

	_, ax = plt.subplots()
	ax.plot()

	plt.title(name + f"_{tp}")
	plt.plot([i+1 for i in range(len(values))], lasted_for)
	plt.savefig(path)




def main():
	results_path = 'results'
	terms_map, name = parse_case_terms_and_name()
	base_path = os.path.join(results_path, name)

	if not os.path.exists(base_path):
		os.mkdir(base_path)

	analyze_time_plot(terms_map, name, os.path.join(base_path, "client_time.png"), "client")
	analyze_time_plot(terms_map, name, os.path.join(base_path, "server_time.png"), "server")

	analyze_gannt(terms_map, name, os.path.join(base_path, "gannt.png"))


if __name__ == "__main__":
	main()