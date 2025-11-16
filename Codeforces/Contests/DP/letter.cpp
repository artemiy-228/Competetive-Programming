#include <iostream>
#include <string>
#include <vector>

using namespace std;

bool is_upper(char& c){
    return c < 97;
}

int main(){
    string input;
    cin >> input;

    size_t n = input.length();

    vector<int> pref(n, 0);
    vector<int> suff(n, 0);

    for(int i = 1; i < n; i++){
        pref[i] = pref[i - 1] + 1 * !is_upper(input[i - 1]);
    }

    for(int i = n - 2; i >= 0; i--){
        suff[i] = suff[i + 1] + 1 * is_upper(input[i + 1]);
    }

    int min = 100 * n;

    for(int i = 0; i < n; i++){
        if (min > pref[i] + suff[i])
            min = pref[i] + suff[i];
    }
    cout << min << '\n';
}
