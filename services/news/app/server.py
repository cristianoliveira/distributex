from flask import Flask, render_template, make_response, request
import logging

from app.api import fetch_top_news, fetch_details_for_story
from app.repository import ArticleRepo

server = Flask(__name__)
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

repo = ArticleRepo()

@server.route("/news")
def root():
    news = fetch_top_news()
    return render_template("index.html", posts=news[:10])

@server.route("/news/more")
def load_more_news():
    offset = int(request.args.get('offset', default=0))
    limit = int(request.args.get('limit', default=10))
    print("@@@@@@ offset, limit: ", offset, limit)
    news = fetch_top_news()

    if offset > len(news):
        offset = 0
        limit = 0

    if limit > len(news):
        limit = len(news)

    return render_template(
        "_load_more.html",
        posts=news[offset:limit],
        offset=offset+limit,
        limit=limit+limit
    )

@server.route("/news/story/<item_id>/link")
def story_link(item_id):
    details = fetch_details_for_story(item_id)

    favorited = repo.get_item(item_id)
    if favorited is None:
        favorited = False

    return render_template("_story_link.html", details=details, favorited=favorited)

@server.route("/news/story/<item_id>/details")
def story_details(item_id):
    details = fetch_details_for_story(item_id)
    return render_template("_story_details.html", details=details)

@server.route("/news/story/<item_id>/favorite")
def story_favorite(item_id):
    favorited = repo.get_item(item_id)

    is_new_fav = favorited is None
    if is_new_fav:
        favorited = True
        repo.insert_item(item_id, favorited)

    if not is_new_fav:
        favorited = repo.update_item(item_id, not favorited)

    resp = make_response(render_template("_story_favorite.html", favorited=favorited))

    resp.headers['HX-Trigger'] = 'news-favorited' if favorited else 'news-unfavorited'

    return resp
