// Integrated with rust backend

import 'package:news_app/model/news.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

// attempts to fetch news when called
Future<List<News>> fetchNews() async {
  final response = await http.get(
    Uri.parse('http://localhost:3000/articles'), // Change if running on emulator or deployed - this is connection to rust backend output to server
  );

  if (response.statusCode == 200) {
    final List data = json.decode(response.body);
    return data.map((json) => News.fromJson(json)).toList();
  } else {
    throw Exception('Failed to load news');
  }
}


// // Flutter - only 
// import 'package:news_app/model/news.dart';
// import 'dart:convert';
// import 'package:http/http.dart' as http;

// Future<List<News>> fetchNews() async {
//   final response = await http.get(
//     // Uri.parse('https://newsdata.io/api/1/news?apikey=YOUR_API_KEY&q=technology&language=en'),
//     Uri.parse('https://newsdata.io/api/1/news?apikey=pub_f45c08a3ecda4a9f8167c3f8acae5bfd&q=technology&language=en'),
//   );

//   if (response.statusCode == 200) {
//     final data = json.decode(response.body);
//     final List articles = data['results'];
//     return articles.map((json) => News.fromJson(json)).toList();
//   } else {
//     throw Exception('Failed to load news');
//   }
// }

/* // Local version
List<News> postsData = [
  News(
    title: 'news A title',
    body: 'news A body',
  ),
  News(
    title: 'news B title',
    body: 'news B body',
  ),
  News(
    title: 'news C title',
    body: 'news C body',
  ),
  News(
    title: 'news D title',
    body: 'news D body',
  ),
  News(
    title: 'news E title',
    body: 'news E body',
  ),
  News(
    title: 'news F title',
    body: 'news F body',
  ),
];
*/