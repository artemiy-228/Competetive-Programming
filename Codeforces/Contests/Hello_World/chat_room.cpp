#include <bits/stdc++.h>

using namespace std;

int main(){
    int i = 0, j = 0;
    string word, pattern = "hello";
    cin >> word;
    while(i < word.size() && j < pattern.size()){
        if(word[i] == pattern[j]){
            j++;
        }
        i++;
    }
    cout << (j == pattern.size() ? "YES" : "NO") << '\n';
}
