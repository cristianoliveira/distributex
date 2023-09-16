from flask import Flask, render_template
from app.api import fetch_top_news, fetch_details_for_story

server = Flask(__name__)

@server.route("/")
def root():
    news = fetch_top_news()
    return render_template("index.html", posts=news[:10])

@server.route("/story/<item_id>/link")
def story_link(item_id):
    details = fetch_details_for_story(item_id)
    return render_template("_story_link.html", details=details)

@server.route("/story/<item_id>/details")
def story_details(item_id):
    details = fetch_details_for_story(item_id)
    return render_template("_story_details.html", details=details)
