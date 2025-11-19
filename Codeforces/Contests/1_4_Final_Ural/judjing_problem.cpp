#include <bits/stdc++.h>

using namespace std;


int main(){
    int t;
    cin >> t;

    for(int _ = 0; _ < t; _++){
        int size;
        unordered_map<string, int> next_a;
        unordered_map<string, int> next_b;
        vector<int> seq;
        string a, b;
        cin >> size;
        cin >> a >> b;
        next_a[a] = 0;
        next_b[b] = 0;
        vector<int> v;
        v.emplace_back(0);

        for(int i = 1; i < size; i++){
            cin >> a >> b;
            if(next_a.find(a) == next_a.end()){
                next_a[a] = i;
            }
            if(next_b.find(b) == next_b.end()){
                next_b[b] = i;
            }

            v.emplace_back(min(next_a[a], next_b[b]));
        }

        bool is_incr = true;
        for(int i = 1; i < size; i++){
            if (v[i] < v[i - 1]){
                is_incr = false;
                break;
            }
        }
        cout << (is_incr ? "Yes" : "No") << '\n';
    }
}
