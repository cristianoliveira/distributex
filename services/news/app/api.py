import requests

def fetch_top_news():
    response = requests.get('https://hacker-news.firebaseio.com/v0/topstories.json')
    return response.json()

def fetch_details_for_story(story_id):
    response = requests.get(f'https://hacker-news.firebaseio.com/v0/item/{story_id}.json')
    return response.json()
