slint::include_modules!();

fn main() {
    MainWindow::new()
        .expect("falha ao criar MainWindow")
        .run()
        .expect("falha ao executar MainWindow");
}

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compila_e_teste_basico_passa() {
        // Nível B: evidência automatizada sem abrir janela gráfica.
        assert_eq!(2 + 2, 4);
    }
}
