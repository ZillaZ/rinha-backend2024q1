CREATE TABLE clientes (
    id int,
    limite int,
    saldo int
);

create table transacoes (
    id_cliente int,
    tipo char,
    descricao varchar(10),
    realizada_em timestamp with time zone,
    valor int
);

create index transacoes_index on transacoes using hash(id_cliente);

DO $$
BEGIN
  INSERT INTO clientes
  VALUES
    (1, 1000 * 100, 0),
    (2, 800 * 100, 0),
    (3, 10000 * 100, 0),
    (4, 100000 * 100, 0),
    (5, 5000 * 100, 0);
END; $$