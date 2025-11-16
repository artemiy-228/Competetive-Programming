#include <iostream>
#include <string>


using namespace std;


struct X{
    int value;

    X() : value(0) {}

    void operation(string& s){
        if (s == "++X" || s == "X++")
            value++;
        else
            value--;
    }
};

int main(){
    X x;
    int c;
    cin >> c;
    for(int i = 0; i < c; i++){
        string temp;
        cin >> temp;
        x.operation(temp);
    }
    cout << x.value << '\n';
}
