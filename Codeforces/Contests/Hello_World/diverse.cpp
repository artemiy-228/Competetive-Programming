#include <bits/stdc++.h>
#define lli long long int
using namespace std;

int main(){
    int n, k;
    cin >> n >> k;
    vector<pair<int, int>> counter(100, {0, -1});
    int unique_count = 0;

    for(int i = 0; i < n; i++){
        int t;
        cin >> t;
        counter[t - 1].first += 1;
        if (counter[t - 1].second == -1){
            counter[t - 1].second = i;
            unique_count++;
        }
    }

    if (unique_count >= k){
        cout << "YES" << '\n';
        int printed = 0;
        for(int i = 0; i < 100 && printed < k; i++){
            if(counter[i].second != -1){
                cout << counter[i].second + 1 << ' ';
                printed++;
            }
        }
        cout << '\n';
    } else {
        cout << "NO" << '\n';
    }
}
