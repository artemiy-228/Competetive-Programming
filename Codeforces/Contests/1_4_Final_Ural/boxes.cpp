#include <bits/stdc++.h>
#define lli long long int

using namespace std;


int main(){
    lli count;
    cin >> count;
    lli min1 = LLONG_MAX, min2 = LLONG_MAX, min3 = LLONG_MAX;
    for(int i = 0; i < count; i++){
        lli a, b, c;
        cin >> a >> b >> c;
        vector<lli> box = {a, b, c};
        sort(box.begin(), box.end());

        min1 = min(min1, box[0]);
        min2 = min(min2, box[1]);
        min3 = min(min3, box[2]);
    }
    cout << min1 * min2 * min3 << '\n';

}
