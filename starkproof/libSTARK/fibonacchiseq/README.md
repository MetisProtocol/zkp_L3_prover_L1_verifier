# libSTARK-exmaple

## Compliation on macOS:
```
git clone git@github.com:LayerXcom/libSTARK.git
cd libSTARK
make -j8
```

## STARK for Fibonacci sequence

Arguments format:
```
./fibonacchi-seq <A: initial number> <B: initial number>
```
in the root directory of libSTARK.

for example:
```
./fibonacchi-seq 52 9
```

The above execution results in execution of STARK simulation over the fibonacchi sequence.The statement is "I know secret numbers A, B such that the 5th element of the Fibonacci sequence is 131. They are A=52, B=9"
