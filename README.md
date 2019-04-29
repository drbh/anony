# 🕵️‍♂️ Welcome to anony

This program simply replaces sensive data in a dataset with specfied values. This program is intended to be a plug-and-play replacements to slower script based anonomizing applications.

## Features

🐥 [X] Read in CSV as stream from   
🐥 [X] Replace SSN with `XXX-XX-XXX`  
🏗️ [ ] Replace Phone Numbers  
🏗️ [ ] Allow user config for replacement value  
🏗️ [ ] Allow user to hash or randomize cells  
🏗️ [ ] Replacing addresses and name  
🏗️ [ ] Detect possible single-out or correlation issues  

# How to use

```
git clone https://github.com/drbh/anony.git
cd anony
cargo build --release
./target/debug/anony < ./data/example.csv 
```

### cat it
You can also pass `cat` output to the program
```
cat ./data/example.csv | ./target/debug/anony 
```

### save to file
You can also redirect the output to a new csv
```
./target/debug/anony < ./data/example.csv > test.csv
```


# Benchmarking 

In the repo I've included a simple Python3.7 script that does the same intenden SSN replacements. Below you can see the runtimes on a very small 16 row CSV file. 

While this is a very very small scaled test, you can see that `anony` takes less than half the time that the Python script does. Future tests will include much larger files. 

## Python
```bash
(base) drbh2s-MBP:anony drbh2$ time python src/main.py < /data/example.csv 
Lastname,Firstname,SSN,Test1,Test2,Test3,Test4,Final,Grade
Alfalfa,Aloysius,XXX-XX-XXXX,40.0,90.0,100.0,83.0,49.0,D-
Alfred,University,XXX-XX-XXXX,41.0,97.0,96.0,97.0,48.0,D+
Gerty,Gramma,XXX-XX-XXXX,41.0,80.0,60.0,40.0,44.0,C
Android,Electric,XXX-XX-XXXX,42.0,23.0,36.0,45.0,47.0,B-
Bumpkin,Fred,XXX-XX-XXXX,43.0,78.0,88.0,77.0,45.0,A-
Rubble,Betty,XXX-XX-XXXX,44.0,90.0,80.0,90.0,46.0,C-
Noshow,Cecil,XXX-XX-XXXX,45.0,11.0,-1.0,4.0,43.0,F
Buff,Bif,XXX-XX-XXXX,46.0,20.0,30.0,40.0,50.0,B+
Airpump,Andrew,XXX-XX-XXXX,49.0,1.0,90.0,100.0,83.0,A
Backus,Jim,XXX-XX-XXXX,48.0,1.0,97.0,96.0,97.0,A+
Carnivore,Art,XXX-XX-XXXX,44.0,1.0,80.0,60.0,40.0,D+
Dandy,Jim,XXX-XX-XXXX,47.0,1.0,23.0,36.0,45.0,C+
Elephant,Ima,XXX-XX-XXXX,45.0,1.0,78.0,88.0,77.0,B-
Franklin,Benny,XXX-XX-XXXX,50.0,1.0,90.0,80.0,90.0,B-
George,Boy,XXX-XX-XXXX,40.0,1.0,11.0,-1.0,4.0,B
Heffalump,Harvey,XXX-XX-XXXX,30.0,1.0,20.0,30.0,40.0,C

real	0m0.034s
user	0m0.023s
sys		0m0.009s
```

## Rust
```bash
(base) drbh2s-MBP:anony drbh2$ time ./target/debug/anony < /data/example.csv 
Lastname,Firstname,SSN,Test1,Test2,Test3,Test4,Final,Grade
Alfalfa,Aloysius,XXX-XX-XXXX,40.0,90.0,100.0,83.0,49.0,D-
Alfred,University,XXX-XX-XXXX,41.0,97.0,96.0,97.0,48.0,D+
Gerty,Gramma,XXX-XX-XXXX,41.0,80.0,60.0,40.0,44.0,C
Android,Electric,XXX-XX-XXXX,42.0,23.0,36.0,45.0,47.0,B-
Bumpkin,Fred,XXX-XX-XXXX,43.0,78.0,88.0,77.0,45.0,A-
Rubble,Betty,XXX-XX-XXXX,44.0,90.0,80.0,90.0,46.0,C-
Noshow,Cecil,XXX-XX-XXXX,45.0,11.0,-1.0,4.0,43.0,F
Buff,Bif,XXX-XX-XXXX,46.0,20.0,30.0,40.0,50.0,B+
Airpump,Andrew,XXX-XX-XXXX,49.0,1.0,90.0,100.0,83.0,A
Backus,Jim,XXX-XX-XXXX,48.0,1.0,97.0,96.0,97.0,A+
Carnivore,Art,XXX-XX-XXXX,44.0,1.0,80.0,60.0,40.0,D+
Dandy,Jim,XXX-XX-XXXX,47.0,1.0,23.0,36.0,45.0,C+
Elephant,Ima,XXX-XX-XXXX,45.0,1.0,78.0,88.0,77.0,B-
Franklin,Benny,XXX-XX-XXXX,50.0,1.0,90.0,80.0,90.0,B-
George,Boy,XXX-XX-XXXX,40.0,1.0,11.0,-1.0,4.0,B
Heffalump,Harvey,XXX-XX-XXXX,30.0,1.0,20.0,30.0,40.0,C

real	0m0.024s
user	0m0.018s
sys		0m0.005s
```


### Example Data
example1.csv - [https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html](https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html)  
example2.csv - [https://dlptest.com/sample-data/](https://dlptest.com/sample-data/)  
