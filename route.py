#!/home/awth/miniconda3/bin/python3
# fastest sqrt (thanks to C stl):
from math import sqrt
from sys import argv
if __name__ == "__main__":
    source = argv[1]
    target = argv[2]
    # distance function
    def r(node1: tuple, node2: tuple):
        return sqrt((node1[0] - node2[0])**2 \
                + (node1[1] - node2[1])**2  
                + (node1[2] - node2[2])**2)
    # IO
    nodes = {} # {label: (x, y, z)}
    edges = []
    with open('nodes.csv', 'r') as f:
        lines = f.read().split('\n')
        # first line is header, last line is empty
        for line in lines[1:-1]:
            label, x, y, z = line.split(',')
            nodes[label] = (float(x), float(y), float(z))
    with open('edges.csv', 'r') as f:
        lines = f.read().split('\n')
        # same header problem
        for line in lines[1:-1]:
            In, Out = line.split(',')
            edges.append((In, Out))
    # Testing
    assert ('cat', 'h') in edges
    assert ('h', 'd') in edges
    assert ('b', 'a') in edges
    assert ('h', 'a') in edges

    # Bellman-Ford
    inf = float('inf')
    dist = dict(zip(nodes.keys(), [inf for _ in nodes.values()]))
    child = {}
    dist[source] = 0
    for label, point in nodes.items():
        for edge in edges:
            u, v, w = edge[0], edge[1], r(nodes[edge[0]], nodes[edge[1]])
            if dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                child[u] = v
    # reachable
    if dist[target] < inf:
        print("->", source, end=' ')
        while source != target:
            print("->", child[source], end=' ')
            source = child[source]
        print(dist[target])
    else:
        print("unreachable")


