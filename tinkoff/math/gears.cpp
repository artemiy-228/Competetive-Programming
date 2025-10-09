#include <iostream>
#define lli long long int

using namespace std;

lli gcd(lli x, lli y){
    while (x > 0 && y > 0){
        if (x > y){
            x = x % y;
        }
        else{
            y = y % x;
        }
    }
    return max(x, y);
}

int main(){
    lli x, y;
    cin >> x >> y;
    lli c = gcd(x, y);
    cout << x * y / c << '\n';
}
