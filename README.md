# stack-alg-sim

Simulation of stack algorithms, originally defined by Mattson et al. 1970.  For the LRU distance (called reuse distance by a share of the literature), 
more efficient algorithms include Olken 1981 and Ding and Zhong PLDI 2003 (Zhong et al. TOPLAS 2009). 

```
stack-alg-sim
├─ .git
├─ .gitattributes
├─ .gitignore
├─ LICENSE
├─ README.md
├─ lru-stack
│  ├─ Cargo.toml
│  └─ src
│     └─ lib.rs
├─ lru-vec
│  ├─ Cargo.toml
│  └─ src
│     ├─ lib.rs
│     └─ main.rs
├─ olken
│  ├─ Cargo.toml
│  └─ src
│     └─ lib.rs
└─ test-cases
   ├─ Cargo.toml
   └─ src
      └─ lib.rs

```