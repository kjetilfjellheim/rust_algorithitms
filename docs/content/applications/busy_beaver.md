## Description
Busy beaver

## Installation
Copy file from releases into /usr/bin

You might need to run chmod uga+x /usr/bin/busy-beaver

## Configuration

max_interations: Max iterations to run. 
programs: A arraylist of programs to run

Example
```
max_iterations = 100000000000

programs = [
"1RB1RE_0LC1RA_1RZ1LD_0LE0LA_0RA1LF_1LC1RC"
]
```
## Run
```
cat <Configuration file> | busy-beaver 
```




