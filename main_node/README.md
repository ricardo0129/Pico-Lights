# Main Node

Since I want to have multiple lights controlled by a single controller node. This is a rust tcp server that will accept worker connections
and process them accordingly. Managing the state of active workers and distributing commands to them. Some of my patterns may require
coordinated actions across multiple lights, so this server will help facilitate that. It's written in rust because why not.
