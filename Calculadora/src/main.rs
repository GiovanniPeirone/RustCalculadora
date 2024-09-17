fn Calcular(n1 : i32, n2: i32, operador: &str ) -> i32
{
    if (operador == "suma")
    {
        let suma = n1 + n2;
        return suma;
    }
}

fn main() {
    let n1 = 4;
    let n2 = 5;

    let suma = Calcular(n1, n2, "suma");

    println!("El numero es {suma}");

}
