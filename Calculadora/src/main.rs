fn calcular(n1 : i32, n2: i32, operador: &str ) -> i32
{
    if operador == "SUMA" {
        let suma = n1 + n2;
        return suma;
    }
    if operador == "RESTA"{
        let resta = n1 - n2;
        return resta;
    }
    if operador == "MULTI"{
        let multi = n1 * n2;
        return multi;
    }else {
        return 0;
    }
}

fn main() {
    let n1 = 4;
    let n2 = 5;

    let reusltado = calcular(n1, n2, "suma");

    println!("El numero es {reusltado}");

}
