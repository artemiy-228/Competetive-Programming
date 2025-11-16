#include <bits/stdc++.h>

using namespace std;


int main(){
    int n, m;
    cin >> n >> m;

    bool is_black = true;

    for(int i = 0; i < n; i++){
        for(int j = 0; j < m; j++){
            char c;
            cin >> c;
            if (c != 'W' &&  c != 'B' && c != 'G'){
                is_black = false;
                break;
            }
        }
        if(!is_black){
            break;
        }
    }
    is_black ? cout << "#Black&White" : cout << "#Color";
}
