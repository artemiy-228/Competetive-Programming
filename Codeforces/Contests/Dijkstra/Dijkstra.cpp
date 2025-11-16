#include <iostream>
#include <set>
#include <vector>
#include <stack>


#define lli_MAX 1'000'000'000'000ll
#define lli long long int

struct Edge{
    lli to;
    lli weight;

    Edge(lli to, lli weight) : to(to), weight(weight){}
};

using namespace std;


void dijkstra(vector<vector<Edge>>& graph, vector<lli>& distances, vector<bool>& visited, vector<lli>& prev, lli start) {
    lli n = graph.size();
    distances[start] = 0;

    for(int _ = 0; _ < n; _++){
        lli v = -1;

        for(int j = 0; j < n; j++){
            if (!visited[j] && (v == -1 || distances[j] < distances[v])){
                v = j;
            }
        }


        if (distances[v] == lli_MAX)
            return;
        visited[v] = true;

        for(auto& e: graph[v]){
            if (distances[v] + e.weight < distances[e.to]){
                prev[e.to] = v;
                distances[e.to] = distances[v] + e.weight;
            }
        }
    }

}


int main(){
    lli n, m;

    cin >> n >> m;

    vector<vector<Edge>> graph(n, vector<Edge>());

    for(lli i = 0; i < m; i++){
        lli from, to, weight;
        cin >> from >> to >> weight;

        graph[from - 1].emplace_back(to - 1, weight);
        graph[to - 1].emplace_back(from - 1, weight);
    }

    vector<bool> visited(n, false);
    vector<lli> distances(n, lli_MAX);
    vector<lli> prev(n, -1);

    dijkstra(graph, distances, visited, prev, 0);

    stack<lli> path;

    for(lli v = n - 1; v != -1; v = prev[v])
        path.push(v);

    if (distances[n - 1] == lli_MAX) {
        cout << -1 << '\n';
        return 0;
    }

    while(!path.empty()){
        cout << path.top() + 1 << ' ';
        path.pop();
    }
    cout << '\n';
}
