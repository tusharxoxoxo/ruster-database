# ruster-database

now i am going to write some wisdom here, like i am trying to this in this repo, it is very important for me to remember what the heck i have done, or else all of this will go in vain, it's time to cultivate a really nice habit of documenting what ever u are doing, so here we go

firstly we have created our .env file and filled it with DATABASE_URL=postgresql://postgres@localhost/ruster-database


funny name --> ruster-database

but first we need to run diesel setup:-
diesel setup does 2 very important thinggs
1. firstly is reads that .env file connects with postgres and actually creates our database in our postgres server whose url we have given in .env file
2. it creates the migration directory in our project.

a migration (in context of automatic schema migration) is a unit of sql that describes both how to apply and reverse a schema change. 
diesel cli can help us setup the file structure for a new migration

for achieving what we have stated in the above line first we have to 
    1. diesel migration generate videos 
    diesel have created a directory inside our migrations directory that's prefixed with a date time and then postfix with what we named that migration which in our case is videos
    migrations are run in coronological order specified by the timestamps on the directory name, beyond that we can arrange our migration however we like
    i am going to a separate migration for each of my tables like videos, users, views, i can also put all these tables inside a single migration but i like to keep separated, a personal choice, who knows must be because of the youtube tutorial guy did the same

