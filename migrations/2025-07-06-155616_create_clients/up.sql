-- Your SQL goes here
CREATE TABLE "clients"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"limit" INTEGER NOT NULL,
	"balance" INTEGER NOT NULL
);

CREATE TABLE "transactions"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"client_id" INTEGER NOT NULL,
	"value" INTEGER NOT NULL,
	"kind" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"timestamp" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("client_id") REFERENCES "clients"("id")
);

