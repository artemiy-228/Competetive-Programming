#include <bits/stdc++.h>

using namespace std;

int main(){
    int len;
    cin >> len;
    vector<int> stones(len, 0);
    string row;
    cin >> row;

    for(int i = 1; i < len; i++){
        stones[i] = row[i - 1] == row[i] ? stones[i - 1] + 1 : 0;
    }
    int counter = 0;
    for(int i = 0; i < len; i++){
        counter += !(stones[i] == 0);
    }
    cout << counter << '\n';
}
