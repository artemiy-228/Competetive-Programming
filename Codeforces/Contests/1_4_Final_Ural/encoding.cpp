#include <bits/stdc++.h>
#define lli long long int

using namespace std;


int main(){
    lli x;
    cin >> x;

    stack<int> s;
    s.push(x % 128);
    x /= 128;
    while(x > 0){
        s.push(x % 128 + 128);
        x /= 128;
    }
    while(!s.empty()){
        cout << s.top() << ' ';
        s.pop();
    }
    cout << '\n';
}
