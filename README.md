# CDR Toolkit

Un toolkit creado para la asignatura Comunicación de Datos y Redes, cursada en la UIB.

Es una potente y rápida CLI que ayuda a realizar los siguientes cálculos básicos que se
necesitan en la asignatura:

- Entropía
- Caracterización de un código
- Eficiencias de mecanimos de control de flujo
- Eficiencias de mecanismos de control de errores
- Eficiencia de redes Ethernet
- Eficiencia de redes WiFi
- Checksums
   
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

La primera opción nos permite realizar el cálculo de la Entropía de una fuente a partir de las probabilidades de sus símbolos a través de las siguiente fórmula:

$$ H(X) = \sum_{i=1}^{S} p_i \cdot \log_2 \frac{1}{p_i} $$

donde $S$ representa el tamaño del alfabeto y $p_i$ la probabilidad de cada símbolo. 

### Advertencia

Esta herramienta aún está en "beta", esto significa que podría tener algunos errores
o algún tipo de bug. Se recomienda encarecidamente seguir las instrucciones que
proporciona el programa.

También se agradecería que si se encuentra algún tipo de bug o error, se me comunicara
para poder solucionarlo lo antes posible.

