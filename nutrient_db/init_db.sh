#!/bin/bash

sed "s?#dir#?$(pwd)?g" ./init_db.txt | sqlite3 nutrients.db;
