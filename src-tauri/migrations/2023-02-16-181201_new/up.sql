CREATE TABLE product (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    price REAL NOT NULL,
    tva REAL NOT NULL DEFAULT 0,
    type TEXT NOT NULL DEFAULT 'product'
);


CREATE TABLE client (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    addresse TEXT
);


CREATE TABLE seller (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    addresse TEXT
);


CREATE TABLE invoice (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    total REAL NOT NULL,
    createdAt DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sellerId INTEGER NOT NULL,
    CONSTRAINT command_sellerId_fkey FOREIGN KEY (sellerId) REFERENCES Seller (id) ON DELETE CASCADE ON UPDATE CASCADE
);


CREATE TABLE invoiceItem (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    productId INTEGER NOT NULL,
    invoiceId INTEGER NOT NULL,
    quantity BIGINT NOT NULL,
    stockId INTEGER NOT NULL,
    CONSTRAINT invoiceItem_productId_fkey FOREIGN KEY (productId) REFERENCES Product (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT invoiceItem_invoiceId_fkey FOREIGN KEY (invoiceId) REFERENCES invoice (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT invoiceItem_stockId_fkey FOREIGN KEY (stockId) REFERENCES stockMouvement (id) ON DELETE NO ACTION ON UPDATE CASCADE
);


CREATE TABLE command (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL,
    createdAt DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    clientId INTEGER NOT NULL,
    CONSTRAINT invoice_clientId_fkey FOREIGN KEY (clientId) REFERENCES Client (id) ON DELETE CASCADE ON UPDATE CASCADE
);


CREATE TABLE commandItem (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    productId INTEGER NOT NULL,
    commandId INTEGER NOT NULL,
    stockId INTEGER NOT NULL,
    quantity BIGINT NOT NULL,
    CONSTRAINT commandItem_productId_fkey FOREIGN KEY (productId) REFERENCES Product (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT commandItem_commandId_fkey FOREIGN KEY (commandId) REFERENCES command (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT commandItem_stockId_fkey FOREIGN KEY (stockId) REFERENCES stockMouvement (id) ON DELETE NO ACTION ON UPDATE CASCADE
);


CREATE TABLE stockMouvement (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    model TEXT NOT NULL,
    quantity BIGINT NOT NULL,
    productId INTEGER NOT NULL,
    CONSTRAINT stockMouvement_productId_fkey FOREIGN KEY (productId) REFERENCES Product (id) ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX product_name_key ON product(name);

-- CreateIndex
CREATE UNIQUE INDEX invoiceItem_stockId_key ON invoiceItem(stockId);

-- CreateIndex
CREATE UNIQUE INDEX commandItem_id_key ON commandItem(id);

-- CreateIndex
CREATE UNIQUE INDEX commandItem_stockId_key ON commandItem(stockId);
