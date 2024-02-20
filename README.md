# CDR Toolkit 🛜

A toolkit created for the course Data Communication and Networks, from UIB.

It is a blazingly fast CLI that helps you to do some basic calculations that are needed in the course:

1. [Entropy](#1-entropy-calculation)
2. [Code characterization](#2-code-characterization)
3. [Efficiency of flux control mechanisms](#3-efficiency-of-flux-control-mechanisms)
4. [Efficiency of error control mechanisms](#4-efficiency-of-error-control-mechanisms)
5. [Ethernet networks efficiency](#5-ethernet-networks-efficiency)
6. [WiFi Networks Efficiency](#6-wi-fi-networks-efficiency)
7. [Checksums](#7-checksums-calculation-deactivated)
   
## How can you use it? 🛠️

#### Using the executable

Using CDR Toolkit is as easy as downloading the executable, opening a terminal where you have the file downloaded and 
running it, regardless of the platform you are on.

#### Using Cargo

Cargo is the package manager for Rust, you can find more information about it [here](https://doc.rust-lang.org/cargo/).
To use cdr_toolkit with cargo, we can install it with the following command:

```bash
cargo install cdr_toolkit
```

Once installed, we can run it directly by typing the following command:

```bash
cdr_toolkit
```

## Functionalities ⚙️

Once the program is executed, we will be presented with a menu with all the options that we can choose:

### 1. Entropy calculation

It allows us to calculate the Entropy of a source from the probabilities of its symbols through the following formula:

$$ H(X) = \sum_{i=1}^{S} p_i \cdot \log_2 \frac{1}{p_i} $$

where $S$ represents the size of the alphabet and $p_i$ the probability of each symbol.

### 2. Code characterization

Make the basic calculations of the characterization of a source. These are the average length of the symbols, 
the Kraft inequality and the Efficiency. These are calculated through the following formulas:

**Average length of the code words**

$$ L = \sum_{i=1}^{S} p_i \cdot l_i $$

**Kraft's inequality**

$$ K = \sum_{i=1}^{S} 2^{-l_i} $$

**Source code efficiency**

$$ Efficiency = \frac{H(X)}{L} $$

### 3. Efficiency of flux control mechanisms

Calculates the efficiency of the desired mechanism, using the correct formula in each case depending on the data 
that is available.

$$ a = \frac{t_{prop}}{t_{frame}} = \frac{d \cdot R}{L \cdot v_{prop}} $$

#### Stop & Wait

$$ \eta_{SW} = \frac{t_{frame}}{t_{frame} + 2 \cdot t_{ptop}} = \frac{1}{1 + 2a} $$

#### Slippery Window

$$ \eta_{SW} = \frac{N \cdot t_{frame}}{t_{frame} + 2 \cdot t_{ptop}} = \frac{N}{1 + 2a} \text{, if } N \leq 2a + 1 \\ $$

$$\eta_{SW} = 1 \text{, si } N \geq 2a + 1 $$

### 4. Efficiency of error control mechanisms

Calculates the efficiency of the desired mechanism, using the correct formula in each case depending on the data that is available.

$$ a = \frac{t_{prop}}{t_{frame}} = \frac{d \cdot R}{L \cdot v_{prop}} $$

#### Stop & Wait ARQ

$N = 1$

$$ \eta = \frac{1 - p}{1 + 2a} $$

#### Go Back N

$N \leq 2^{k} - 1$

$$ \eta = \frac{1 - p}{1 + 2a \cdot p}, N \geq 2a + 1 $$

$$ \eta = \frac{N(1 - p)}{(2a + 1)(1 - p + N \cdot p)}, N < 2a + 1 $$

#### Selective Reject

$N \leq 2^{k - 1}$

$$ \eta = 1 - p, N \geq 2a + 1 $$

$$\eta = \frac{N(1 - p)}{2a + 1}, N < 2a + 1 $$

### 5. Ethernet networks efficiency

Calculates the efficiency of an Ethernet network given the following formulas.

$$ A = \left(1 - \frac{1}{N}\right)^{N - 1} $$

$$ \eta = \frac{1}{1 + \frac{\tau_b}{L \cdot A}} $$

where $L$ is the size of the frames in bits and $\tau_b$ a standardized value:

- 512 in Ethernet 10BASE-T and Ethernet 100BASE-T4
- 4096 in Ethernet 1000BASE-T

### 6. Wi-Fi Networks Efficiency

Calculates the efficiency of a Wi-Fi network.

### 7. Checksums calculation (Deactivated)

Calculates a checksum from the given numbers. The calculation is done using the complement to 1 sum method.

## Warning ⚠️

This tool is still in "beta", this means that it could have some errors or bugs. 
It is strongly recommended to follow the instructions provided by the program.

It would also be appreciated if any kind of bug or error is found, it is communicated to me
to be able to solve it as soon as possible.
