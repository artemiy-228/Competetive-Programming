#include <iostream>
#include <vector>
#include <queue>
using namespace std;

void bfs(vector<vector<int>>& graph, vector<pair<int, int>>& visited, queue<int>& q){
    while (!q.empty()){
        int value = q.front();
        q.pop();
        for (int i: graph[value]){
            visited[i].first += 1;
            if (visited[i].second == -1 && visited[i].first * 2 >= graph[i].size()){
                q.push(i);
                visited[i].second = visited[value].second + 1;
            }
        }
    }



}

int main() {
    int n, m, k;
    cin >> n >> m >> k;

    vector<vector<int>> graph(n);

    for (int i = 0; i < m; i++) {
        int u, v;
        cin >> u >> v;
        u--; v--;
        graph[u].push_back(v);
        graph[v].push_back(u);
    }

    vector<pair<int, int>> visited(n, {0, -1});
    vector<int> minimum(n, -1);

    vector<int> starts;
    queue<int> infected;
    for(int i = 0; i < k; i++){
        int t;
        cin >> t;
        t -= 1;
        starts.push_back(t);
        infected.push(t);
        visited[t] = {0, 0};
    }
    bfs(graph, visited, infected);
    for (int i = 0; i < n; i++) {
        cout << visited[i].second << ' ';
    }
    cout << '\n';

}
