import sqlite3

class ArticleRepo:
    def __init__(self):
        db = sqlite3.connect('articles.db', check_same_thread=False)
        db.execute("""
        CREATE TABLE IF NOT EXISTS favorites
        (item_id INTEGER PRIMARY KEY AUTOINCREMENT,
        favorited BOOLEAN DEFAULT FALSE)
        """)
        self.db = db

    def get_item(self, item_id: int) -> bool | None:
        curs = self.db.cursor()
        transaction = curs.execute("SELECT * FROM favorites WHERE item_id = ?", (item_id,))

        try:
            _, favorited = transaction.fetchone()
            return favorited
        except TypeError:
            return None

    def insert_item(self, item_id: int, favorited: bool) -> None:
        self.db.execute("INSERT INTO favorites (item_id, favorited) VALUES (?, ?)", (item_id, favorited))
        self.db.commit()

    def update_item(self, item_id: int, favorited: bool) -> bool:

        self.db.execute("UPDATE favorites SET favorited = ? WHERE item_id = ?", (favorited, item_id))
        self.db.commit()

        return favorited
