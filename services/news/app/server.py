from flask import Flask, render_template, make_response, request
import logging

from app.api import fetch_top_news, fetch_details_for_story
from app.repository import ArticleRepo

server = Flask(__name__)
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

repo = ArticleRepo()

PAGINATION_LIMIT = 3

@server.route("/")
def root():
    return render_template("index.html")

@server.route("/news")
def stories_list():
    return render_template("_stories_list.html")

@server.route("/news/favorites")
def favorited_list():
    favorites = repo.get_favorites()

    return render_template(
        "_favorired_list.html",
        stories_ids=favorites,
        offset=0,
        limit=0,
    )

@server.route("/news/favorites/<item_id>")
def favorited_details(item_id):
    details = fetch_details_for_story(item_id)

    favorited = repo.get_item(item_id)
    if favorited is None:
        favorited = False

    if not favorited:
        return ""

    return render_template("_favorited_details.html", details=details, favorited=favorited)

@server.route("/news/more")
def load_more_news():
    offset = int(request.args.get('offset', default=0))
    limit = int(request.args.get('limit', default=PAGINATION_LIMIT))

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

    resp.headers['HX-Trigger'] = 'unfavorited-event-{}'.format(item_id) if not favorited else 'favorited-event'

    return resp
