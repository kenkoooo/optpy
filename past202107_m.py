INF = 1e15

N, C = map(int, input().split())
A = list(map(int, input().split()))

V = N * 2 + 2
S = N * 2
T = N * 2 + 1

G = [[] for _ in range(V)]


def add_edge(u, v, capacity, cost):
    G[u].append([v, capacity, cost, len(G[v])])
    G[v].append([u, 0, -cost, len(G[u]) - 1])


def bellman_ford(s):
    dist = [INF] * V
    dist[s] = 0
    pv = [0] * V
    pe = [0] * V
    while True:
        update = False
        for v in range(V):
            if dist[v] == INF:
                continue
            for i in range(len(G[v])):
                next, capacity, cost, _ = G[v][i]
                if capacity > 0 and dist[next] > dist[v] + cost:
                    dist[next] = dist[v] + cost
                    update = True
                    pv[next] = v
                    pe[next] = i
        if not update:
            break

    return dist, pv, pe


def calc_min_cost_flow(s, t, f):
    result = 0
    while f > 0:
        dist, pv, pe = bellman_ford(s)
        if dist[t] == INF:
            return INF
        flow = f
        v = t
        while v != s:
            flow = min(flow, G[pv[v]][pe[v]][1])
            v = pv[v]
        result += flow * dist[t]
        f -= flow
        v = t
        while v != s:
            G[pv[v]][pe[v]][1] -= flow
            rev = G[pv[v]][pe[v]][3]
            G[v][rev][1] += flow
            v = pv[v]
    return result


for i in range(N):
    add_edge(S, i, 1, 0)
    add_edge(i, T, 1, C)


for i in range(N):
    for j in range(i + 1, N):
        add_edge(i, N + j, 1, abs(A[i] - A[j]))


for j in range(N):
    add_edge(N + j, T, 1, 0)

ans = calc_min_cost_flow(S, T, N)
print(ans)
