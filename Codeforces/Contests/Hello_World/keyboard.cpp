#include <bits/stdc++.h>
#define lli long long int

using namespace std;

int contains(string& str, char& c){
    for(int i = 0; i < str.length(); i++){
        if(str[i] == c){
            return i;
        }
    }
    return -1;
}


int main(){
    string row1 = "qwertyuiop";
    string row2 = "asdfghjkl;";
    string row3 = "zxcvbnm,./";
    char c;
    cin >> c;
    string message;
    cin >> message;

    int step = c == 'R' ? -1 : +1;

    for(int i = 0; i < message.length(); i++){
        int i1 = contains(row1, message[i]);
        int i2 = contains(row2, message[i]);
        int i3 = contains(row3, message[i]);
        if (i1 != -1){
            cout << row1[i1 + step];
        } else if (i2 != -1){
            cout << row2[i2 + step];
        } else if (i3 != -1){
            cout << row3[i3 + step];
        }
    }
    cout << '\n';
}
