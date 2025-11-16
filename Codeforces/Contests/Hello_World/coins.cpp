#include <bits/stdc++.h>
#define lli long long int

using namespace std;


int main(){
    lli i, a, b, c, n;
    cin >> i;
    for(int _ = 0; _ < i; _++){
        cin >> a >> b >> c >> n;
        lli summary = a + b + c + n;
        if (summary % 3 == 0 && summary / 3 >= max(a, max(b, c)))
            cout << "YES" << '\n';
        else
            cout << "NO" << '\n';
    }
}
