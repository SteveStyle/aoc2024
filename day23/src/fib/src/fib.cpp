#include <stdio.h>

constexpr long long fib(long long n)
{
    return n <= 1 ? n : fib(n - 2) + fib(n - 1);
}

constexpr long long FIBRESULT = fib(26);

int main()
{
    printf("%lld\n", FIBRESULT);
    return 0;
}