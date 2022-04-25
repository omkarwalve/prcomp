import flask
import urllib3
import requests
from flask import Flask, request
import json

from relv import rvs
from recom import genValidQueries,rcSort

# Setup flask server
app = flask.Flask(__name__)
http = urllib3.PoolManager()
devServerDomain = 'http://localhost:8000'

@app.route("/")
def home():
    return "hello"

# Frontend
@app.route("/rc/<uid>")
def getRecommendations(uid):
    # Fetch user clicks
    response = requests.get(f'{devServerDomain}/clk/{uid}')
    user_clicks = []
    if response.status_code == 200:
        # user clicks stored in user_clicks
        user_clicks = response.json()['data']

    x = genValidQueries(user_clicks)
    with open('x.json', 'r') as f:
        return rcSort(json.load(f), user_clicks)


def mQuery_result(querylist):
    # { queries : [ "realme+pro", "jbl+flip" .... ] }

    payload = { "queries": querylist }

    return requests.post(f'{devServerDomain}/q/elx', json=payload).json()

@app.route('/rlv/<query>')
def rlv_sort(query):
    json = KWE(query)
    return rvs(json)

def KWE(qry):
   return requests.get(f'{devServerDomain}/q/elx/{qry}').json()

if __name__ == "__main__":
    app.run(port=8051, debug=True)
