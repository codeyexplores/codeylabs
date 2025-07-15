import 'package:flutter/material.dart';
import 'package:news_app/data/sample_data.dart';
import 'package:news_app/model/news.dart';
import 'package:news_app/widget/custom_search_delegate.dart';
import 'package:news_app/widget/post_list.dart';

void main() {
  runApp(const Homepage());
}

class Homepage extends StatefulWidget {
  const Homepage({super.key});

  @override
  State<Homepage> createState() => _HomePageState();
}

class _HomePageState extends State<Homepage> {
  late Future<List<News>> _newsFuture;

  @override
  void initState() {
    super.initState();
    _newsFuture = fetchNews(); // the cost
  }
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.lightBlueAccent),
      ),
      home: Scaffold(
        appBar: AppBar(
          title: const Text('All News'),
          actions: [
            FutureBuilder<List<News>>(
              future: _newsFuture,
              builder: (context, snapshot) {
                if (!snapshot.hasData) return Container();
                return IconButton(
                  icon: Icon(Icons.search),
                  onPressed: () {
                    showSearch(
                      context: context,
                      delegate: CustomSearchDelegate(snapshot.data!),
                    );
                  },
                );
              },
            ),
          ],
        ),
        body: FutureBuilder<List<News>>(
          future: _newsFuture,
          builder: (context, snapshot) {
            if (snapshot.connectionState == ConnectionState.waiting) {
              return const Center(child: CircularProgressIndicator());
            } else if (snapshot.hasError) {
              return Center(child: Text('Error: ${snapshot.error}'));
            } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
              return const Center(child: Text('No news found.'));
            } else {
              return PostList(posts: snapshot.data!);
            }
          },
        ),
      ),
    );
  }
}
