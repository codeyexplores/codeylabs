// this is the search bar widget to filter news

import 'package:flutter/material.dart';
import 'package:news_app/model/news.dart';
import 'package:news_app/widget/post_list.dart';

// abstract class called SearchDelegate
class CustomSearchDelegate extends SearchDelegate<void> {
  CustomSearchDelegate(this.posts);

  final List<News> posts;
  List<News> localResult = []; // tracks and update search results

  // builds widget on top right - button
  @override
  List<Widget>? buildActions(BuildContext context) => [
    IconButton(
      onPressed: () => query.isEmpty ? close(context, null): query = "", // if user input empty, close, else filled, clear searchbar
      icon: Icon(Icons.clear),)
  ];

  // builds widget on top left
  @override
  Widget? buildLeading(BuildContext context) => 
    IconButton(
      onPressed: () => close(context, null), 
      icon: Icon(Icons.arrow_back_rounded),
    );

  // responsible for showing results
  @override
  Widget buildResults(BuildContext context) {
    return localResult.isEmpty
      ? Center(child: Text("No Results", style: TextStyle(fontSize: 24)))
      : PostList(posts: localResult);
  }

  // responsible for search functionality
  @override
  Widget buildSuggestions(BuildContext context) {
    localResult = posts.where((News post) {
      final String title = post.title.toLowerCase();
      final String body = post.description.toLowerCase();
      final String input = query.toLowerCase();

      return title.contains(input) || body.contains(input);      
    }).toList();
  return localResult.isEmpty ? Center(
    child: Text(
            'No Result',
            style: TextStyle(fontSize: 24),
          ),
    ) : PostList(posts: localResult);
  }
}
