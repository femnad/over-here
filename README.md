over-here
=========

A notification server to listen for [desktop notifications](https://developer.gnome.org/notification-spec/).

Built on [notify-rust](https://github.com/hoodie/notify-rust)'s experimental server component.

Why?
====

I don't know, [Ratpoison](http://www.nongnu.org/ratpoison/) and [StumpWM](https://stumpwm.github.io/) do not seem to play well with [dunst](https://github.com/knopwob/dunst) when dunst's notification window is overlayed with a window when it is active, making that notification and any subsequent ones invisible until all the windows are hidden or all the dunst notifications are cleared. Obviously, this is a minor imperfection, but this seemed like a decent workaround for a second.

How?
===

Run `over-here` and send messages via `notify-send`

What's the Catch?
=================

Requires [dzenotify](https://github.com/femnad/dzenotify) to be on the `PATH`.
