// Importante: Per connettersi a un qualsiasi DB Access bisogna PRIMA installare
//la versione opportuna di AccessDatabaseEngine
// da https://www.microsoft.com/en-us/download/details.aspx?id=54920

extern crate odbc;
// Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
extern crate env_logger;
extern crate odbc_safe;
use odbc::*;

//#[allow(unused_imports)]        //permetto l'uso di use std altrimenti da errore
use odbc_safe::AutocommitOn;

// const DB: &str = "./EsempioStudenti.accdb";

pub fn access() -> Vec<String> {
    //select conessione ok or error
    connect()
}

//funzione connessione + diagnostica
fn connect() -> Vec<String> {
    //assegna a env un ambiente odbc
    let env = create_environment_v3().map_err(|e| e.unwrap()).unwrap();

    //Importante: per leggere da excel usare:
    // "Driver={Microsoft Excel Driver (*.xls, *.xlsx, *.xlsm, *.xlsb)};DBQ=c:/my/file.xls"

    /* codice che recupero la stringa della cnn da cli*/
    //let mut mybuffer = String::new();
    // println!("Inserire la stringa di connessione [esempio: Driver={{Microsoft Access Driver (*.mdb, *.accdb)}}; DBQ=c:/my/file.accdb]: ");
    // io::stdin().read_line(&mut buffer).unwrap();
    // //\r\n= unico modo per windows per riconoscere solo l'invio senza stringa  e per default attiv l'avviso
    // if buffer.eq(&String::from("\r\n")) {
    //  mybuffer = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=./PRES3000_N25_PIANTA_ORGANICA.mdb;".to_owned(); // to_owned() converte &str in String
    // }

    /* ALTERNATIVA = uso stringa di connessione costante*/
    let mybuffer = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=./PRES3000_N25_PIANTA_ORGANICA.mdb;".to_owned(); // to_owned() converte &str in String

    //attivo la conenessione passando il buffer
    let conn = env
        .connect_with_connection_string(&mybuffer)
        .expect("Fallita query");
    execute_statement(&conn)
}

//questa funzione esegue una query qualsiasi attenzione viene utilizzato 'env = life time delle fuzione e dell'oggetto
fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Vec<String> {
    // Crea un nuovo Statement (comando SQL vuoto) a partire dalla connessione passata come parametro.
    let stmt = Statement::with_parent(conn).expect("errore nella creazione Statement sql");

    //todo: queyr su tabella ma non funzione per i caratteri accentati tipo Scirè Calabrisotto Andrea oppure De Podestà Emanuela
    let sql_text = "SELECT * FROM DIPENDENTI ORDER BY DIPENDENTI.ID_DIPEN_lng;".to_owned();

    // eseguo la query scritta da linea di comando con exec_direct()
    // costrutto match con due rami: Data(statement) e NoData(_)
    let tabella: Vec<String> = match stmt
        .exec_direct(&sql_text)
        .expect("impossibile eseguire query")
    {
        // Se ci sono dati, li stampo in output
        Data(mut stmt) => {
            let mut tabella: Vec<String> = vec![]; //crea un vettore vuoto

            // Stampo ogni colonna separando con uno spazio
            let cols = stmt.num_result_cols().unwrap_or(0); // numero di colonne recuperato con il ciclo while
                                                            // finche' c'e' qualche (Some) riga (la riga si prende con statement.fetch())
            while let Some(mut cursor) = stmt.fetch().expect("errore fetch") {
                let mut riga = String::new();

                // per ogni colonna (le colonne partono da 1 per ODBC)
                for i in 1..=cols {
                    // intervallo di numeri 1,2,3,...,numero_colonne. l'= serve per includere l'estremo superiore dell'intervallo

                    let cella = match cursor.get_data::<&str>(i as u16).unwrap_or(Some("errore")) {
                        // se ci sono dati nella cella, stampo con uno spazio davanti per separare.
                        Some(val) => format!("{};", val),
                        // se non ci sono dati nella cella, stampo la parola NULL (valore nullo)
                        None => format!("null;"),
                    };

                    riga.push_str(&cella);
                }
                // vado a capo
                riga.push('\n');
                tabella.push(riga); //riempie la tabella un record alla volta
            }
            return tabella;
        }
        // Se la query non restituisce dati, stampo una stringa.
        NoData(_) => {
            vec![]
        }
    };
    println!("Dati ottenuti");
    // Restituisco Ok al chiamante. Se si arriva qui, è andato tutto bene (Ok!)
    tabella
}

//test record
#[cfg(test)]
mod tests {
    use crate::access::connect;

    #[test]
    fn it_works() {
        let result = connect();
        assert!(result.len() > 0);
    }
}
