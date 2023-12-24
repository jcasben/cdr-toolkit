# CDR Toolkit

Un toolkit creado para la asignatura Comunicación de Datos y Redes, cursada en la UIB.

Es una potente y rápida CLI que ayuda a realizar los siguientes cálculos básicos que se
necesitan en la asignatura:

1. [Entropía](#1-cálculo-de-entropía)
2. [Caracterización de un código](#2-caracterización-de-un-código)
3. [Eficiencias de mecanimos de control de flujo](#3-eficiencias-mecanismos-de-control-de-flujo)
4. [Eficiencias de mecanismos de control de errores](#4-eficiencias-mecanismos-de-control-de-errores)
5. [Eficiencia de redes Ethernet](#5-eficiencia-redes-ethernet)
6. [Eficiencia de redes WiFi](#6-eficiencia-redes-wifi)
7. [Checksums](#7-cálculo-de-checksums)
   
## ¿Como lo puedo usar?

#### A través del ejecutable

Usar CDR Toolkit es tan fácil como descargar el ejecutable, abrir una terminal donde tengas descargado el archivo y ejecutarlo, independientemente de la plataforma en la que te encuentres.

#### A través de cargo

Cargo es el administrador de paquetes de Rust, puedes consultar más información sobre él [aquí](https://doc.rust-lang.org/cargo/).
Para utilizar cdr_toolkit con cargo, podemos instalarlo con el siguiente comando:

```shell
cargo install cdr_toolkit
```

Una vez instalado, podemos ejecutarlo directamente escribiendo el siguiente comando:

```shell
cdr_toolkit
```

## Opciones de uso

Una ejecutado el programa, se nos presentará un menú con todas las opciones que podremos elegir:

### 1. Cálculo de Entropía

Nos permite realizar el cálculo de la Entropía de una fuente a partir de las probabilidades de sus símbolos a través de las siguiente fórmula:

$$ H(X) = \sum_{i=1}^{S} p_i \cdot \log_2 \frac{1}{p_i} $$

donde $S$ representa el tamaño del alfabeto y $p_i$ la probabilidad de cada símbolo.

### 2. Caracterización de un código

Realiza los cálculos básicos de la caracterización de una fuente. Estos son la Longitud media de los símbolos, la desigualdad de Kraft y la Eficiencia.
Estos se calculas a través de las siguientes fórmulas:

**Longitud media de las palabras código**

$$ L = \sum_{i=1}^{S} p_i \cdot l_i $$

**Desigualdad de Kraft**

$$ K = \sum_{i=1}^{S} 2^{-l_i} $$

**Eficiencia del código fuente**

$$ Eficiencia = \frac{H(X)}{L} $$

### 3. Eficiencias mecanismos de control de flujo

Calcula la eficiencia del mecanismo deseado, utilizando la fórmula correcta en cada caso dependiendo de los datos que se tengan.

$$ a = \frac{t_{prop}}{t_{frame}} = \frac{d \cdot R}{L \cdot v_{prop}} $$

#### Stop & Wait

$$ \eta_{SW} = \frac{t_{frame}}{t_{frame} + 2 \cdot t_{ptop}} = \frac{1}{1 + 2a} $$

#### Ventana Deslizante

$$ \eta_{SW} = \frac{N \cdot t_{frame}}{t_{frame} + 2 \cdot t_{ptop}} = \frac{N}{1 + 2a} \text{, si } N \leq 2a + 1 \\ $$

$$\eta_{SW} = 1 \text{, si } N \geq 2a + 1 $$

### 4. Eficiencias mecanismos de control de errores

Calcula la eficiencia del mecanismo deseado, utilizando la fórmula correcta en cada caso dependiendo de los datos que se tengan.

$$ a = \frac{t_{prop}}{t_{frame}} = \frac{d \cdot R}{L \cdot v_{prop}} $$

#### Stop & Wait ARQ

$N = 1$

$$ \eta = \frac{1 - p}{1 + 2a} $$

#### Go Back N

$N \leq 2^{k} - 1$

$$ \eta = \frac{1 - p}{1 + 2a \cdot p}, N \geq 2a + 1 $$

$$ \eta = \frac{N(1 - p)}{(2a + 1)(1 - p + N \cdot p)}, N < 2a + 1 $$

#### Rechazo Selectivo

$N \leq 2^{k - 1}$

$$ \eta = 1 - p, N \geq 2a + 1 $$

$$\eta = \frac{N(1 - p)}{2a + 1}, N < 2a + 1 $$

### 5. Eficiencia redes Ethernet

Calcula la eficiencia de una red Ethernet dadas las siguientes fórmulas.

$$ A = \left(1 - \frac{1}{N}\right)^{N - 1} $$

$$ \eta = \frac{1}{1 + \frac{\tau_b}{L \cdot A}} $$

donde $ L $ es el tamaño de las tramas en bits y $ \tau_b $ un valor estandarizado:

- 512 en Ethernet 10BASE-T y Ethernet 100BASE-T4
- 4096 en Ethernet 1000BASE-T

### 6. Eficiencia redes WiFi

Calcula la eficiencia de una red WiFi y la muestra a través de un gráfico. Este se guardará en directorio actual en un archivo con el nombre plot.png.

### 7. Cálculo de checksums

Calcula un checksum a partir de los números dados. El cálculo se realiza utilizando el método de la suma en complemento a 1.

## Advertencia

Esta herramienta aún está en "beta", esto significa que podría tener algunos errores
o algún tipo de bug. Se recomienda encarecidamente seguir las instrucciones que
proporciona el programa.

También se agradecería que si se encuentra algún tipo de bug o error, se me comunicara
para poder solucionarlo lo antes posible.