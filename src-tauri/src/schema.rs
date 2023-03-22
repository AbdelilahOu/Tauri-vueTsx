// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Integer,
        name -> Text,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        addresse -> Nullable<Text>,
    }
}

diesel::table! {
    command (id) {
        id -> Integer,
        status -> Text,
        createdAt -> Timestamp,
        clientId -> Integer,
    }
}

diesel::table! {
    commandItem (id) {
        id -> Integer,
        productId -> Integer,
        commandId -> Integer,
        stockId -> Integer,
        quantity -> BigInt,
    }
}

diesel::table! {
    invoice (id) {
        id -> Integer,
        total -> Float,
        createdAt -> Timestamp,
        sellerId -> Integer,
    }
}

diesel::table! {
    invoiceItem (id) {
        id -> Integer,
        productId -> Integer,
        invoiceId -> Integer,
        quantity -> BigInt,
        stockId -> Integer,
    }
}

diesel::table! {
    product (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        price -> Float,
        tva -> Float,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    seller (id) {
        id -> Integer,
        name -> Text,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        addresse -> Nullable<Text>,
    }
}

diesel::table! {
    stockMouvement (id) {
        id -> Integer,
        date -> Timestamp,
        model -> Text,
        quantity -> BigInt,
        productId -> Integer,
    }
}

diesel::joinable!(commandItem -> command (commandId));
diesel::joinable!(commandItem -> stockMouvement (stockId));
diesel::joinable!(invoiceItem -> invoice (invoiceId));
diesel::joinable!(invoiceItem -> stockMouvement (stockId));

diesel::allow_tables_to_appear_in_same_query!(
    client,
    command,
    commandItem,
    invoice,
    invoiceItem,
    product,
    seller,
    stockMouvement,
);
